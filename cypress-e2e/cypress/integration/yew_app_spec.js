describe('The Yew app', () => {
    beforeEach('Visit the app', () => {
        cy.visit('/');
    });

    it('Loads successfully', () => {
        cy.get('.container').should('exist');
    });

    it('Has a counter display that is initially zero', () => {
        cy.get('[data-cy="counter-display"]').should('have.text', '0');
    });

    it('Has an add-one button that adds one', () => {
        cy.get('[data-cy="add-one-button"]').click();
        cy.get('[data-cy="counter-display"]').should('have.text', '1');
    });

    it('Has an add-two button that adds two', () => {
        cy.get('[data-cy="add-two-button"]').click();
        cy.get('[data-cy="counter-display"]').should('have.text', '2');
    });

    it('Has an add-three button that adds three', () => {
        cy.get('[data-cy="add-three-button"]').click();
        cy.get('[data-cy="counter-display"]').should('have.text', '3');
    });

    it('Has a times-two button that multiplies by two', () => {
        cy.get('[data-cy="times-two-button"]').click();
        cy.get('[data-cy="counter-display"]').should('have.text', '0');

        cy.get('[data-cy="add-three-button"]').click();
        cy.get('[data-cy="times-two-button"]').click();
        cy.get('[data-cy="counter-display"]').should('have.text', '6');
    });

    it('Has a reset button that resets the counter', () => {
        cy.get('[data-cy="counter-display"]').should('have.text', '0');
        cy.get('[data-cy="add-three-button"]').click();
        cy.get('[data-cy="counter-display"]').should('have.text', '3');
        cy.get('[data-cy="reset-button"]').click();
        cy.get('[data-cy="counter-display"]').should('have.text', '0');
    });

    it('Should handle complex sequences of actions', () => {
        cy.get('[data-cy="counter-display"]').should('have.text', '0');
        cy.get('[data-cy="add-three-button"]').click();
        cy.get('[data-cy="counter-display"]').should('have.text', '3');
        cy.get('[data-cy="reset-button"]').click();
        cy.get('[data-cy="counter-display"]').should('have.text', '0');
        cy.get('[data-cy="add-two-button"]').click();
        cy.get('[data-cy="counter-display"]').should('have.text', '2');
        cy.get('[data-cy="add-three-button"]').click();
        cy.get('[data-cy="counter-display"]').should('have.text', '5');
        cy.get('[data-cy="times-two-button"]').click();
        cy.get('[data-cy="counter-display"]').should('have.text', '10');
        cy.get('[data-cy="add-one-button"]').click();
        cy.get('[data-cy="counter-display"]').should('have.text', '11');
        cy.get('[data-cy="reset-button"]').click();
        cy.get('[data-cy="counter-display"]').should('have.text', '0');
    });
});
